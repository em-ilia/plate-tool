function copy_screenshot(el) {
	html2canvas(el).then((canvas) => {
		console.log("Copying image to clipboard");
		let data = canvas.toDataURL();

		const textArea = document.createElement("textarea");
		textArea.value = data;

		document.body.prepend(textArea);
		textArea.select();

		document.execCommand('copy');
		document.body.removeChild(textArea);
	});
}

function copy_screenshot_dest() {
	let plate = document.getElementsByClassName("dest_plate")[0];
	copy_screenshot(plate);
}
function copy_screenshot_src() {
	let plate = document.getElementsByClassName("source_plate")[0];
	copy_screenshot(plate);
}
