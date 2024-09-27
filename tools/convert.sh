#!/bin/bash

shopt -s nocaseglob

find . -iname '*.dds' | while read file; do
    # Nome do arquivo sem extensão
    output="${file%.[dD][dD][sS]}.png"

    # Verificar se o arquivo PNG já existe e pular a conversão se estiver presente
    if [ ! -f "$output" ]; then
        # Usar o magick para converter o arquivo .dds em .png
        ./tools/magick "$file" "$output"
        echo "Convertido: $file -> $output"
    else
        echo "Ignorando: $output (já existe)"
    fi
done