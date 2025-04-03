/*
const fs = require('fs')
export function read_file_js(path) {
	//return fs.readFileSync(path, { encoding: 'utf8' });
	try {
		const data = fs.readFileSync(path, 'utf8');
		console.log(data);
		return data;
	} catch (err) {
		console.error(err);
	}
}
*/

export function starting() {
	return 'starting';
}

/*
module.exports = {
	read_file_js
}
*/