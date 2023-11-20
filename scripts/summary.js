const fs = require("fs")

// first argument is the polyform's percolation probability
// second argument is the polyform's number of elements
// third argument is the number of polyforms to analyze

const size = process.argv[2];


function get_summary(size, probability) {
	let files = fs.readdirSync("../../analysis/"+size+"/"+probability+"/").filter((file) => file.endsWith(".txt_betti.txt"))

	let total_holes = [];
	for(let file of files) {
		let betti = fs.readFileSync("../../analysis/"+size+"/"+probability+"/"+file)
		let holes = betti.toString("ascii").replace("\n","").trim().split(" ");
		//console.log(holes);

		for(let k=0; k<holes.length; k++) {
			if(total_holes[k] == null) total_holes[k] = 0;
			total_holes[k] += parseInt(holes[k]);
		}
	}

	return total_holes.map(total => total/files.length);
}

console.log(`Perc. Prob.\t${['Connected Comp.','B0','B1','B2'].join('\t')}`);
for (let probability of fs.readdirSync("../../analysis/"+size)) {
	console.log(`${probability}\t${get_summary(size, probability).join('\t')}`);
}
