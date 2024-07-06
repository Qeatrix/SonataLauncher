import App from "@/ui/App"
// if ("paintWorklet" in CSS) {
// 	CSS.paintWorklet.addModule(
// 		"https://www.unpkg.com/css-houdini-squircle@0.3.0/squircle.min.js"
// 	);
// }

document.getElementById('app')!.append(
	...<>
		<App />
	</>
)
