# ihome-iot-server
This is a server for iot thoght to be ran at a RPI

Por el momento
correr -> `export LD_LIBRARY_PATH=$PWD/vosk-api/vosk-linux-aarch64-0.3.45:$LD_LIBRARY_PATH`

Antes de correr el server en RPI

Vosk-api descargas por OS: https://github.com/alphacep/vosk-api/releases


Este proyecto necesita descargar por separado el modelo de Qwen con los siguientes comandos:

`curl -LsSf https://hf.co/cli/install.sh | bash`

`hf download Qwen/Qwen2.5-1.5B-Instruct-GGUF qwen2.5-1.5b-instruct-q4_k_m.gguf --local-dir .`
