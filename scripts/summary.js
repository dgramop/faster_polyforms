const fs = require("fs")

// first argument is the polyform's number of elements
// second argument is the number of polyforms to analyze

let total_holes = [];
for(let i=0; i<parseInt(process.argv[3]); i++) {
	let betti = fs.readFileSync("../analysis/"+process.argv[2]+"/"+i+".txt_betti.txt")
	let holes = betti.toString("ascii").replace("\n","").trim().split(" ");
	console.log(holes);

	for(let k=0; k<holes.length; k++) {
		if(total_holes[k] == null) total_holes[k] = 0;
		total_holes[k] += parseInt(holes[k]);
	}
}

console.log(total_holes);

