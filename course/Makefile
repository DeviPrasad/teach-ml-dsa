html: ntt32 prt zeta
	echo "HTML build complete."

pdf: ntt32-pdf prt-pdf zeta-pdf
	echo "PDF build complete."

ntt32: ntt32.md ntt.yaml style.css
	pandoc ntt32.md --metadata-file=ntt.yaml --mathjax --standalone --number-sections --css=style.css --syntax-highlighting=tango -f markdown -t html -o ntt32.html

prt: prt.md poly-math.yaml style.css
	pandoc prt.md --metadata-file=poly-math.yaml --mathjax --standalone --number-sections --css=style.css --syntax-highlighting=tango -f markdown -t html -o prt.html

zeta: zeta.md zeta.yaml style.css
	pandoc zeta.md --metadata-file=zeta.yaml --mathjax --standalone --number-sections --css=style.css --syntax-highlighting=tango -f markdown -t html -o zeta.html

ntt32-pdf: ntt32.md ntt.yaml
	pandoc ntt32.md --log=pandoc.log --pdf-engine=xelatex --metadata-file=ntt.yaml --syntax-highlighting=tango -o ntt32.pdf

prt-pdf: prt.md poly-math.yaml
	pandoc prt.md --log=pandoc.log --pdf-engine=xelatex --metadata-file=poly-math.yaml --syntax-highlighting=tango -o prt.pdf

zeta-pdf: zeta.md zeta.yaml
	pandoc zeta.md --log=pandoc.log --pdf-engine=xelatex --metadata-file=zeta.yaml --syntax-highlighting=tango -o zeta.pdf

clean:
	rm -f ntt32.html
	rm -f ntt32.pdf
	rm -f *.aux
	rm -f *.log
	rm -f *.out
