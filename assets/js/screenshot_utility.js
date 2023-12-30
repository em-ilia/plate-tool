function copy_screenshot(el) {
	html2canvas(el).then((canvas) => {
		console.log("Copying image to clipboard");
		canvas.toBlob((b) => {
			try {
				navigator.clipboard.write([
					new ClipboardItem({
						'image/png': b
					})
				]);
			} catch (e) {
				console.error("Failed to copy!");
			}
		});
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
