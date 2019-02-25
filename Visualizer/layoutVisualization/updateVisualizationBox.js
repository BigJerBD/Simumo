function updateVisualizationBox() {
	let visualizationBox = document.getElementById('VisualizationBox');
  let metrics = document.getElementsByClassName('metricSelectionBox')[0];

  let mapTab = document.getElementById('tabs').getElementsByTagName("a")[0];
  let otherTab = document.getElementById('tabs').getElementsByTagName("a")[1];

  let timeValue = $("#flat-slider").slider("value");

  if (mapTab.className == "selected") {
    visualizationBox.children[0].style.visibility = 'visible';
    visualizationBox.children[1].style.visibility = 'hidden';
		updateVisualizationLayer("");
  }
	else if (otherTab.className == "selected") {
    visualizationBox.children[0].style.visibility = 'hidden';
    visualizationBox.children[1].style.visibility = 'visible';
		//nothing more to do for now
  }
}
