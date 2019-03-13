function loadColorGradient(timeLineMin, timeLineMax, gradient)
{
	// <div class="gradientbox" style="background-image: linear-gradient(to right,{% for color in legend -%} {{ 'rgb(' + [color.red, color.green, color.blue]|join(',') + ')'}} {{ "," if not loop.last }} {%- endfor %});">
	// </div>
	//
	// <div class="beginLegendBox">
	// 	{{timeline.min}}
	// </div>
	// <div class="middleLegendBox">
	// 	{{timeline.max/2|round|int}}
	// </div>
	// <div class = "endLegendBox">
	// 	{{timeline.max}}
	// </div>
	let gradientBox = document.getElementById("gradientBox");
	gradientBox.innerHTML = "";
	let gradientStyle = "background-image: linear-gradient(to right,";
	for (let i = 0, length = gradient.length; i < length; i++) {
		gradientStyle += 'rgb(' +
		                 gradient[i].r.toString() + ',' +
										 gradient[i].g.toString() + ',' +
										 gradient[i].b.toString() + ')';
		if(i < length -1) gradientStyle += ',';
	};
	gradientStyle += ')';

	//add color gradient
	gradientBox.innerHTML += `<div class=\"gradientbox\" style=\"${gradientStyle}\">`;
	//add scalar index
	let timeLineMiddle = Math.round(timeLineMax / 2.0)
	gradientBox.innerHTML += `<div class="beginLegendBox">
														${timeLineMin}
													</div>
													<div class="middleLegendBox">
														${timeLineMiddle}
													</div>
													<div class = "endLegendBox">
														${timeLineMax}
													</div>`;
}
