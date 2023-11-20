let files = fs.readdirSync("../../analysis/"+process.argv[2]+"/"+process.argv[3]+"/").filter((file) => file.endsWith(".txt_betti.txt"))

let total_holes = [];
for(let file of files) {
	let betti = fs.readFileSync("../../analysis/"+process.argv[2]+"/"+process.argv[3]+"/"+file)
	let holes = betti.toString("ascii").replace("\n","").trim().split(" ");
	console.log(holes);

	for(let k=0; k<holes.length; k++) {
		if(total_holes[k] == null) total_holes[k] = 0;
		total_holes[k] += parseInt(holes[k]);
	}
}

console.log(total_holes);
