import('../pkg')
	.catch(console.error);

import { Chart } from '../pkg';

const canvas = document.getElementById("canvas");
const fileInput = document.getElementById("file-input");
const fileContentDisplay = document.getElementById("file-content");
const messageDisplay = document.getElementById("message");
const channelList = document.getElementById("channels");
var file_buffer_bytes = null;

fileInput.addEventListener("change", handleFileSelection);

function handleFileSelection(event) {
	const file = event.target.files[0];
	fileContentDisplay.textContent = ""; // Clear previous file content
	messageDisplay.textContent = ""; // Clear previous messages

	// Validate file existence and type
	if (!file) {
		showMessage("No file selected. Please choose a file.", "error");
		return;
	}
	file.type
	if (!file.type.startsWith("application/octet-stream")) {
		showMessage("Unsupported file type. Please select a binary file.", "error");
		return;
	}

	// Read the file
	const reader = new FileReader();
	reader.readAsArrayBuffer(file);
	reader.onload = () => {
		fileContentDisplay.textContent = "File length bytes: " + reader.result.byteLength;
	};
	reader.onloadend = () => {
		// Fill the file buffer to draw a plot
		file_buffer_bytes = new Uint8Array(reader.result);
		//Chart.plot_channels(canvas, file_buffer_bytes, Number(channelList.value));
	}
	reader.onerror = () => {
		showMessage("Error reading the file. Please try again.", "error");
	};
}

// Displays a message to the user
function showMessage(message, type) {
	messageDisplay.textContent = message;
	messageDisplay.style.color = type === "error" ? "red" : "green";
}

channelList.addEventListener("change", (event) => {
	if (!file_buffer_bytes) {
		showMessage("No file selected. Please choose a file.", "error");
		return;
	}

	// Draw plot
	Chart.plot_channels(canvas, file_buffer_bytes, Number(channelList.value));
});