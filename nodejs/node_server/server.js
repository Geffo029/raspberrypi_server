const http = require('http')
const fs = require('fs')


const PORT = 1789
const INFOS_FILE = '/tmp/hwinfos'


const pathAliases = {
	'/': '/index.html'
}


const server = http.createServer((requestObj, responseObj) => {
	//console.log("Requested path:", requestObj.url)

	let url = typeof pathAliases[requestObj.url] === 'undefined' ? requestObj.url : pathAliases[requestObj.url]
	let filePath = url === '/infos' ? INFOS_FILE : __dirname + "/public" + url

	fs.readFile(filePath, (err, content) => {
		if (err) {
			console.log("File not found:", filePath)
			responseObj.statusCode = 404
			responseObj.end()
		} else {
			responseObj.statusCode = 200
			responseObj.write(content)
			responseObj.end()
		}
	})
});


server.listen(PORT, () => {
	console.log("Listening on port " + PORT + " ...")
})



// Fancy way to handle different request urls
// https://gist.github.com/prof3ssorSt3v3/8d9fc6be89d3aefd3ea84b92f923181a#file-server-route-js-L52
// let routes = {
	// "/": function() {}
// }
