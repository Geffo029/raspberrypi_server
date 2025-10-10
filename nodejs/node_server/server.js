const http = require('http')
const fs = require('fs')


const PORT = 1789
const INFOS_FILE = '/tmp/hwinfos'


const server = http.createServer((requestObj, responseObj) => {
	// console.log("Request accepted");

	responseObj.setHeader('Content-Type', 'text/html');	// 'text/plain'...
	let code = 200;

	let path = "";
	switch (requestObj.url) {
		case '/': 
			path = './index.html';
			break;
		case '/about': 
			path = './about.html';
			break;
		case '/about-me': 
			responseObj.setHeader('Location', '/about');
			responseObj.statusCode = 301;
			responseObj.end();
			break;
		// case '/scripts/script.js':
		// 	responseObj.setHeader('Content-Type', 'application/javascript')
		// 	path = '.' + requestObj.url
		// 	break;
		case '/infos':
			responseObj.write(fs.readFileSync(INFOS_FILE, 'utf-8', 'r'))
			//responseObj.write(JSON.stringify(updatedInfos()));
			responseObj.end();
			break;
		default: 
			path = './error.html';
			code = 404;
	}

	//responseObj.writeHead(404, {'Content-Type': 'text/html'});
	responseObj.statusCode = code;

	if (path != "") {
		fs.readFile(path, 'utf8', (error, data) => {
			console.log("Reading file " + path)
			if (error) { 
				console.log("Errore " + error); 
			} else {
				responseObj.write(data);
			}
			responseObj.end();
		});
	}
});


server.listen(PORT, () => {
	console.log("Listening on port " + PORT + " ...")
})
