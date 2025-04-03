# Wasm draw channels plot
* Read data from a little endian binary file
	Example: 44 09 F3 06 D1 05 ..
* Select number of channels
	Construct channels from single bytes to u16  
* Commands
	Build: wasm-pack build --target web
	Run: npm run serve
	Test: wasm-pack test --firefox