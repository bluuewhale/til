const os = require("os");
const _ = require("underscore")
const fs = require("fs")

// module example
arr = [3, 12, 9, 6, 15];

console.log(os.platform())
console.log(os.arch())
console.log(arr[0])
console.log(_.first(arr))
console.log(arr[arr.length -1])
console.log(_.last(arr))

// callback
arr.sort((x, y) => {return x-y})
console.log(arr)

// fs
fs.readFile("./data.txt", {encoding:"utf-8"}, (err, data) => {
	if (err !== null) {
		console.log(err);
	} else {
		console.log(data);
	}
})
