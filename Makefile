jekyll-assets :
	cd engine; wasm-pack build --release
	rm www/dist/*
	cd www; npm run build
	rm jekyll/shopping_solo/*.js
	rm jekyll/shopping_solo/*.wasm
	cp www/dist/*.js www/dist/*.wasm jekyll/shopping_solo/
	cd jekyll; bundle exec jekyll build
