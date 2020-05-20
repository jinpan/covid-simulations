jekyll-assets :
	cd engine; wasm-pack build --release
	rm -r www/dist/*
	cd www; npm run build

	rm -f jekyll/intro/*.js
	rm -f jekyll/intro/*.wasm
	cp www/dist/*.js www/dist/*.wasm jekyll/intro/

	rm -f jekyll/shopping_solo/*.js
	rm -f jekyll/shopping_solo/*.wasm
	cp www/dist/*.js www/dist/*.wasm jekyll/shopping_solo/

	cd jekyll; bundle exec jekyll build
