function parseLogs(logs, unitToSelect) {
  let logsJson = JSON.parse(logs);
  let parsedLogs = []
  logsJson.forEach(function(entry) {
    entry["data"].forEach(function(data) {
      if (data["resolution"] == unitToSelect) {
				let parsedLog = {
	        lon: entry["lon"],
	        lat: entry["lat"],
	        metricType: entry["metric_type"],
	        dataType: data["type"],
	        unit: data["resolution"],
	        value: data["value"],
	        interpolation: data["value"] / $("#flat-slider").slider("option", "max") //normalised value
	      }
	      parsedLogs.push(parsedLog);
      }
    });
  });
  return parsedLogs;
}

function secToTimestamp(sec) {
  let nbrHours = parseInt(sec / 3600.0);
  let nbrMinutes = parseInt((sec - (nbrHours * 3600.0)) / 60.0);
  let nbrSeconds = (sec - (nbrHours * 3600.0)) - (nbrMinutes * 60.0)
  return ("0" + nbrHours).slice(-2) + ":" + ("0" + nbrMinutes).slice(-2) + ":" + ("0" + nbrSeconds).slice(-2);
}

function updateVisualizationBox() {
  $("body").css("cursor", "wait");
  let visualizationBox = document.getElementById('VisualizationBox');

  let metrics = document.getElementsByName('metricSelection');
  let selectedMetric;
  for (var i = 0, length = metrics.length; i < length; i++) {
    if (metrics[i].checked) {
      selectedMetric = metrics[i];
      break;
    }
	 }


    let mapTab = document.getElementById('tabs').getElementsByTagName("a")[0];
    let otherTab = document.getElementById('tabs').getElementsByTagName("a")[1];

    let timeValueBegin = $('#flat-slider').slider("option", "values")[0];
    let timeValueEnd = $('#flat-slider').slider("option", "values")[1];

    if (mapTab.className == "selected") {
      visualizationBox.children[0].style.visibility = 'visible';
      visualizationBox.children[1].style.visibility = 'hidden';

      $.ajax({
        url: "/logs/sherbrooke_sample?min=" + secToTimestamp(timeValueBegin) + "&max=" + secToTimestamp(timeValueEnd),
        cache: false,
        success: function(logs) {
          let parsedLogs = parseLogs(logs,selectedMetric.getAttribute('data-unit'));
          updateVisualizationLayer(parsedLogs);
        }
      });

    } else if (otherTab.className == "selected") {
      visualizationBox.children[0].style.visibility = 'hidden';
      visualizationBox.children[1].style.visibility = 'visible';
      //nothing more to do for now
    }
    $("body").css("cursor", "default");
  }
