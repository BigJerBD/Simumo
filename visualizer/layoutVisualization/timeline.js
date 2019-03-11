// tool used for the timeline: https://simeydotme.github.io/jQuery-ui-Slider-Pips/#installation


function updateTimeline(min, max)
{
	$("#flat-slider").slider({
		min: min,
		max: max,
		range: true,
		values: [min, max],
		change: updateVisualizationBox
	}).slider("pips", {
		rest: "label",
		step: (max - min)/10.0
	});

    document.getElementById("sliderUnit").innerHTML = "sec";
}
