# Conectar Phantom Wallet con Solana Playground 

![Banner](../images/phantomB.jpg)

**Phantom Wallet** es una billetera digital para `Solana` (y otras blockchains) que permite guardar, enviar y recibir tokens, además de conectarse a aplicaciones descentralizadas (`dApps`). Funciona como una extensión de navegador y app móvil, y es la **wallet más usada en el ecosistema Solana**. 

**NOTA**: Como Phantom, también existen otras wallets como `Solflare`, `Backpack` o `Jupyter`. El prodecimiento descrito también funciona para esas y otras wallets, solo es necesario obtener su `Private Key`.

---

Para comenzar es necesario tener la llave privada (`Private Key`) de la extensión de `Phantom Wallet`, lo que se hace mediante los siguientes pasos:

![private key](../images/privatekey.png)

1. Ya abierta la extension de phantom damos clic en el icono de usuario 

2. Entramos a ajustes dando clic en el icono ubicado en la parte inferior izquierda

3. Damos clic la opción `Manage Accounts`

4. Seleccionamos la cuenta que deseamos exportar

5. Clic en `Show Private Key`

6. Introducimos la contraseña

7. Damos clic en la red de Solana

8. Confirmamos que dando clic en el recuadro morado y precionamos continuar

9. Y finalmente copiamos la llave privada.

---

Para poder hacer la importación en el `Solana Playground` es necesario convertir la llave privada en un array de numeros base58. 

> ⚠️ **Antes de continuar es necesario que tengas una sesión iniciada en Google**



Para correr el código se hará uso de `Google Colaboratory`: Clic 👉 [Aquí](https://colab.research.google.com/)

Donde es necesario abrir un nuevo cuaderno presionando primero el `bóton rojo` y posteriormente el `azul`
![colabi](../images/colabi.png)

Lo que nos abrirá un cuaderno de `Python`, donde se debe pegar el siguiente código:

> ⚠️ **Recuerda reemplazar `PEGA TU LLAVE AQUI` por la llave privada**

```
!pip install base58
KEY = "PEGA TU LLAVE AQUI"

import base58
import json

# Decodificar Base58 → bytes
decoded_bytes = base58.b58decode(KEY)

# Convertir a array (lista de enteros)
byte_array = list(decoded_bytes)

# Guardar en un archivo JSON
with open("key_array.json", "w") as f:
  json.dump(byte_array, f, indent=2)
```

Quedando de la siguiente manera:

![colab](../images/colab.png)
Donde solo **es necesario pegar la llave de Phantom en donde dice `PEGA TU LLAVE AQUI`** y ejecutar la celda presionando `SHIFT + ENTER`, o pulsando el bóton `Run all`:

![colab1](../images/colab1.png)

Al finalizar la ejecución nos generará un archivo de nombre `key_array.json`, que descargaremos presionando clic derecho y `Download`.

---

Para continuar es necesario volver al `Playground de solana`, donde es necesario acceder al menú de la wallet que se encuentra en la parte superior derecha:

![wallet-icon](../images/wallet-icon.png)

Damos clic en los tres puntos:

![config-wallet](../images/config-wallet.png)

Y seleccionamos la opcion `Import`:

![add](../images/add.png)

Lo que nos abrirá el selector de archivos. Por último, es necesario elegir el `key_array.json` que descargamos anteriormente.

Y listo, con eso ya tendriamos el address de `Phantom` lista para usar en el `Solana Playground` 🥳 !!!  