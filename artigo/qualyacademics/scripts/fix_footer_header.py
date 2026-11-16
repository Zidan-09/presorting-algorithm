import zipfile
import os

docx_path = r'artigo\qualyacademics\artigo_qualyacademics.docx'
tmp_path = docx_path + '.tmp'

with zipfile.ZipFile(docx_path, 'r') as zin, zipfile.ZipFile(tmp_path, 'w') as zout:
    for item in zin.infolist():
        data = zin.read(item.filename)
        name = item.filename
        if name == 'word/header2.xml':
            data = data.replace(b'000-14300', b'204-220')
        if name == 'word/footer2.xml':
            # Replace marketing text runs with empty strings
            txts = [
                'DESEJA VER UM ARTIGO PRONTO? ',
                'Baixe um artigo completo em PDF, o link está disponível na seguinte página: ',
                'https://www.editoraunisv.com.br/post/politica-publica-de-educacao-em-prisoes-do-tocantins-marcos-historicos-fundamentais ',
                '(procure o link baixar o artigo completo em PDF)',
            ]
            for t in txts:
                if t.encode('utf-8') in data:
                    data = data.replace(t.encode('utf-8'), b'')
        zout.writestr(item, data)

os.replace(tmp_path, docx_path)
print('DOCX fixes applied')
