
ntt32: ntt32.md md.yaml style.css
	pandoc ntt32.md --metadata-file=md.yaml --mathjax --standalone --css=style.css --syntax-highlighting=tango -f markdown -t html -o ntt32.html

ntt32-pdf-prior: ntt32.md md.yaml style.css
	pandoc ntt32.md --log=pandoc.log -o ntt32.tex --pdf-engine=xelatex -M link-citations=true --citeproc --metadata-file=md.yaml --standalone -M link-citations=true --citeproc --template eisvogel --from markdown --syntax-highlighting=idiomatic -V linkcolor=blue -V header-includes:'\usepackage{amsmath,amscd} \usepackage{hyperref} \usepackage{tikz} \usepackage{lscape} \usepackage{everypage} \usepackage[export]{adjustbox} \let\includegraphicsbak\includegraphics \renewcommand*{\includegraphics}[2][]{\includegraphicsbak[frame,#1]{#2}}' -o ntt32.pdf

ntt32-pdf: ntt32.md md.yaml style.css
	pandoc ntt32.md --log=pandoc.log -o ntt32.tex --pdf-engine=xelatex --syntax-highlighting=tango  -M link-citations=true --citeproc --metadata-file=md.yaml --standalone -M link-citations=true --citeproc --template eisvogel --from markdown --syntax-highlighting=idiomatic -V linkcolor=blue -V header-includes:'\usepackage{amsmath,amscd} \usepackage{hyperref} \usepackage{tikz} \usepackage{pdflscape} \usepackage{afterpage} \usepackage[export]{adjustbox} \let\includegraphicsbak\includegraphics \renewcommand*{\includegraphics}[2][]{\includegraphicsbak[frame,#1]{#2}}' -o ntt32.pdf


clean:
	rm -f ntt32.html
	rm -f ntt32.pdf
	rm -f *.aux
	rm -f *.log
	rm -f *.out
