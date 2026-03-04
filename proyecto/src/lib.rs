// ============================================================
// GymTracker — Programa nativo Rust para Solana Playground
// ============================================================
// Archivo único lib.rs que consolida:
//   - state.rs        → structs on-chain con Borsh
//   - initialize_profile → Instrucción 0
//   - log_session        → Instrucción 1
// ============================================================

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

// ============================================================
// SECCIÓN 1: STATE — Estructuras de datos on-chain
// ============================================================

// ----------------------------------------------------------
// Días de entrenamiento según la tabla de Edgar
// El orden del enum define cómo Borsh lo serializa (u8):
//   0 = LunesAnterior
//   1 = MartesPosterior
//   2 = MiercolesAnterior
//   3 = JuevesPosterior
// El cliente JS debe usar exactamente estos valores numéricos.
// ----------------------------------------------------------
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum TrainingDay {
    LunesAnterior,      // 0
    MartesPosterior,    // 1
    MiercolesAnterior,  // 2
    JuevesPosterior,    // 3
}

// ----------------------------------------------------------
// Un ejercicio registrado dentro de una sesión.
//
// ORDEN DE CAMPOS IMPORTANTE para Borsh:
// El cliente JS debe serializar exactamente en este orden:
//   1. name        (String → u32 length + bytes UTF-8)
//   2. reps_serie_1 (u8)
//   3. reps_serie_2 (u8)
//   4. peso_rir1_x10 (u16 little-endian)
//   5. peso_rir0_x10 (u16 little-endian)
// ----------------------------------------------------------
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct ExerciseLog {
    pub name: String,        // máx ~50 chars para no inflar el MAX_SIZE
    pub reps_serie_1: u8,
    pub reps_serie_2: u8,
    pub peso_rir1_x10: u16,  // Peso * 10 para evitar floats. Ej: 52.5kg → 525
    pub peso_rir0_x10: u16,
}

// ----------------------------------------------------------
// Sesión completa de entrenamiento — vive en una PDA propia.
// PDA seeds: ["sesion", owner_pubkey, sesion_numero (4 bytes LE)]
//
// ORDEN DE CAMPOS para Borsh (el cliente debe respetar este orden
// si alguna vez lee/escribe manualmente):
//   owner, training_day, timestamp, duracion_minutos,
//   calificacion, ejercicios, sesion_numero
// ----------------------------------------------------------
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WorkoutSession {
    pub owner: Pubkey,              // 32 bytes
    pub training_day: TrainingDay,  //  1 byte  (enum u8)
    pub timestamp: i64,             //  8 bytes
    pub duracion_minutos: u16,      //  2 bytes
    pub calificacion: u8,           //  1 byte  (1–5 💪)
    pub ejercicios: Vec<ExerciseLog>, // dinámico
    pub sesion_numero: u32,         //  4 bytes
}

impl WorkoutSession {
    /// MAX_SIZE — espacio máximo reservado en la account.
    ///
    /// Desglose campo a campo:
    ///   32  → Pubkey (owner)
    ///    1  → TrainingDay (enum serializado como u8)
    ///    8  → i64 (timestamp)
    ///    2  → u16 (duracion_minutos)
    ///    1  → u8  (calificacion)
    ///    4  → u32 (prefix de longitud del Vec por Borsh)
    ///   25  → máximo de ejercicios por sesión
    ///         Cada ExerciseLog:
    ///           4 (prefix String) + 50 (chars) + 1 + 1 + 2 + 2 = 60 bytes
    ///   25 * 60 = 1500
    ///    4  → u32 (sesion_numero)
    ///
    /// Total = 32+1+8+2+1+4+1500+4 = 1552
    /// Añadimos margen del 10% → 1552 + 156 = 1708
    pub const MAX_SIZE: usize = 1708;
}

// ----------------------------------------------------------
// Perfil de usuario — UNA account por wallet.
// PDA seeds: ["perfil", owner_pubkey]
// ----------------------------------------------------------
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UserProfile {
    pub owner: Pubkey,          // 32 bytes
    pub total_sesiones: u32,    //  4 bytes
    pub is_initialized: bool,   //  1 byte
}

impl UserProfile {
    /// 32 + 4 + 1 = 37 bytes exactos
    pub const MAX_SIZE: usize = 37;
}

// ============================================================
// SECCIÓN 2: ENTRYPOINT
// ============================================================

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if data.is_empty() {
        msg!("❌ No se recibió ninguna instrucción");
        return Err(ProgramError::InvalidInstructionData);
    }

    // El primer byte es el discriminador de instrucción
    let (discriminator, rest) = data.split_first().unwrap();

    msg!("GymTracker | instrucción: {}", discriminator);

    match discriminator {
        0 => process_initialize_profile(program_id, accounts),
        1 => process_log_session(program_id, accounts, rest),
        _ => {
            msg!("❌ Instrucción desconocida: {}", discriminator);
            Err(ProgramError::InvalidInstructionData)
        }
    }
}

// ============================================================
// SECCIÓN 3: INSTRUCCIÓN 0 — initialize_profile
// ============================================================
// Crea la account UserProfile para el usuario.
// Se llama UNA SOLA VEZ por wallet.
//
// Accounts esperadas (en orden):
//   [0] user           → signer, writable
//   [1] profile_pda    → writable (se crea aquí)
//   [2] system_program → readonly
// ============================================================

fn process_initialize_profile(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let user           = next_account_info(accounts_iter)?;
    let profile_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // El usuario debe firmar
    if !user.is_signer {
        msg!("❌ El usuario debe firmar la transacción");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Derivar PDA y obtener bump
    let seeds: &[&[u8]] = &[b"perfil", user.key.as_ref()];
    let (profile_pda, bump) = Pubkey::find_program_address(seeds, program_id);

    // MEJORA: Verificar que la account recibida coincide con la PDA derivada
    if profile_pda != *profile_account.key {
        msg!("❌ La profile_account no coincide con la PDA esperada");
        msg!("   Esperada: {}", profile_pda);
        msg!("   Recibida: {}", profile_account.key);
        return Err(ProgramError::InvalidArgument);
    }

    // Verificar que no esté ya inicializada (evita doble init)
    if !profile_account.data.borrow().iter().all(|&b| b == 0) {
        msg!("❌ El perfil ya fue inicializado");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    msg!("Creando perfil para: {}", user.key);

    // MEJORA: Rent exempt garantizado — usamos minimum_balance
    // que ya incluye la exención de rent por defecto en Solana 1.8+
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(UserProfile::MAX_SIZE);

    // Crear la account con invoke_signed usando bump verificado
    invoke_signed(
        &system_instruction::create_account(
            user.key,
            profile_account.key,
            lamports,
            UserProfile::MAX_SIZE as u64,
            program_id,
        ),
        &[user.clone(), profile_account.clone(), system_program.clone()],
        // MEJORA: usamos el bump exacto devuelto por find_program_address
        &[&[b"perfil", user.key.as_ref(), &[bump]]],
    )?;

    // Escribir datos iniciales
    let profile = UserProfile {
        owner: *user.key,
        total_sesiones: 0,
        is_initialized: true,
    };
    profile.serialize(&mut &mut profile_account.data.borrow_mut()[..])?;

    msg!("✅ Perfil inicializado | owner: {}", user.key);
    Ok(())
}

// ============================================================
// SECCIÓN 4: INSTRUCCIÓN 1 — log_session
// ============================================================
// Registra una sesión de entrenamiento completa.
// Crea una nueva PDA por sesión usando el número de sesión
// como seed → cada registro es único e inmutable on-chain.
//
// Accounts esperadas (en orden):
//   [0] user            → signer, writable
//   [1] profile_pda     → writable (ya existe)
//   [2] session_pda     → writable (se crea aquí)
//   [3] system_program  → readonly
//
// SERIALIZACIÓN del campo `data` (después del discriminador 0x01):
//   El cliente JS debe serializar LogSessionArgs con Borsh
//   en este orden exacto:
//     1. training_day      (u8 — valor del enum)
//     2. duracion_minutos  (u16 LE)
//     3. calificacion      (u8)
//     4. ejercicios        (u32 LE longitud + elementos)
//        Cada elemento:
//          - name: u32 LE longitud + bytes UTF-8
//          - reps_serie_1: u8
//          - reps_serie_2: u8
//          - peso_rir1_x10: u16 LE
//          - peso_rir0_x10: u16 LE
// ============================================================

// Args que el cliente envía como bytes Borsh
#[derive(BorshDeserialize, Debug)]
pub struct LogSessionArgs {
    pub training_day: TrainingDay,      // u8
    pub duracion_minutos: u16,
    pub calificacion: u8,               // 1–5 💪
    pub ejercicios: Vec<ExerciseLog>,   // Vec serializado por Borsh
}

fn process_log_session(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // MEJORA: Deserializar con manejo explícito de error
    // Si el cliente serializa mal el struct, aquí lo detectamos
    let args = LogSessionArgs::try_from_slice(data).map_err(|e| {
        msg!("❌ Error deserializando LogSessionArgs: {:?}", e);
        msg!("   Verifica que el cliente use Borsh con los campos en el orden correcto");
        ProgramError::InvalidInstructionData
    })?;

    // --- Validaciones de negocio ---
    if args.calificacion < 1 || args.calificacion > 5 {
        msg!("❌ La calificación debe ser entre 1 y 5 💪, recibido: {}", args.calificacion);
        return Err(ProgramError::InvalidArgument);
    }

    if args.ejercicios.is_empty() {
        msg!("❌ Debes registrar al menos un ejercicio");
        return Err(ProgramError::InvalidArgument);
    }

    if args.ejercicios.len() > 25 {
        msg!("❌ Máximo 25 ejercicios por sesión, recibidos: {}", args.ejercicios.len());
        return Err(ProgramError::InvalidArgument);
    }

    // --- Extraer accounts ---
    let accounts_iter   = &mut accounts.iter();
    let user            = next_account_info(accounts_iter)?;
    let profile_account = next_account_info(accounts_iter)?;
    let session_account = next_account_info(accounts_iter)?;
    let system_program  = next_account_info(accounts_iter)?;

    if !user.is_signer {
        msg!("❌ El usuario debe firmar la transacción");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // --- Leer y validar el perfil ---
    let mut profile = UserProfile::try_from_slice(&profile_account.data.borrow())
        .map_err(|_| {
            msg!("❌ No se pudo leer el UserProfile — ¿fue inicializado?");
            ProgramError::UninitializedAccount
        })?;

    if !profile.is_initialized {
        msg!("❌ El perfil no está inicializado. Ejecuta initialize_profile primero.");
        return Err(ProgramError::UninitializedAccount);
    }

    if profile.owner != *user.key {
        msg!("❌ Este perfil no pertenece al firmante");
        return Err(ProgramError::IllegalOwner);
    }

    let sesion_numero = profile.total_sesiones;

    // --- Derivar PDA de la sesión ---
    let sesion_num_bytes = sesion_numero.to_le_bytes(); // u32 → 4 bytes LE
    let seeds: &[&[u8]] = &[b"sesion", user.key.as_ref(), sesion_num_bytes.as_ref()];
    let (session_pda, bump) = Pubkey::find_program_address(seeds, program_id);

    // MEJORA: Verificar bump explícitamente
    if session_pda != *session_account.key {
        msg!("❌ La session_account no coincide con la PDA esperada");
        msg!("   Sesión número: {}", sesion_numero);
        msg!("   Esperada: {}", session_pda);
        msg!("   Recibida: {}", session_account.key);
        return Err(ProgramError::InvalidArgument);
    }

    // --- Timestamp del bloque actual ---
    let clock = Clock::get()?;
    let timestamp = clock.unix_timestamp;

    // --- Crear la account de sesión (rent exempt garantizado) ---
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(WorkoutSession::MAX_SIZE);

    invoke_signed(
        &system_instruction::create_account(
            user.key,
            session_account.key,
            lamports,
            WorkoutSession::MAX_SIZE as u64,
            program_id,
        ),
        &[user.clone(), session_account.clone(), system_program.clone()],
        &[&[b"sesion", user.key.as_ref(), sesion_num_bytes.as_ref(), &[bump]]],
    )?;

    // --- Serializar y guardar la sesión ---
    let session = WorkoutSession {
        owner: *user.key,
        training_day: args.training_day,
        timestamp,
        duracion_minutos: args.duracion_minutos,
        calificacion: args.calificacion,
        ejercicios: args.ejercicios,
        sesion_numero,
    };
    session.serialize(&mut &mut session_account.data.borrow_mut()[..])?;

    // MEJORA: checked_add previene overflow en contadores de larga vida
    profile.total_sesiones = profile.total_sesiones
        .checked_add(1)
        .ok_or_else(|| {
            msg!("❌ Overflow en total_sesiones — límite u32 alcanzado");
            ProgramError::InvalidAccountData
        })?;

    profile.serialize(&mut &mut profile_account.data.borrow_mut()[..])?;

    msg!(
        "✅ Sesión #{} | Día: {:?} | {}min | {}💪 | {} ejercicios",
        sesion_numero,
        session.training_day,
        session.duracion_minutos,
        session.calificacion,
        session.ejercicios.len(),
    );

    Ok(())
}
