let minMaxByMetric = {};
function getMinAndMaxOfLog(log, metric)
{
	if(minMaxByMetric[metric] == undefined)
	{
		let logJson = JSON.parse(log);
		let min = Number.POSITIVE_INFINITY;
		let max = Number.NEGATIVE_INFINITY;
		logJson.forEach(function(entry) {
			entry["data"].forEach(function(data) {
				min = Math.min(data["value"], min);
				max = Math.max(data["value"], max);
			});
		});
		minMaxByMetric[metric] = [min,max];
		return [min,max];
	}
	else {
		return minMaxByMetric[metric];
	}
}

function parseLog(log, unitToSelect, max) {
  let logJson = JSON.parse(log);
  let parsedLog = []
  logJson.forEach(function(entry) {
    entry["data"].forEach(function(data) {
      if (data["resolution"] == unitToSelect) {
        let logEntry = {
          lon: entry["lon"],
          lat: entry["lat"],
          metricType: entry["metric_type"],
          dataType: data["type"],
          unit: data["resolution"],
          value: data["value"],
          interpolation: data["value"] / max //normalised value
        }
        parsedLog.push(logEntry);
      }
    });
  });
  return parsedLog;
}

function secToTimestamp(sec) {
  let nbrHours = parseInt(sec / 3600.0);
  let nbrMinutes = parseInt((sec - (nbrHours * 3600.0)) / 60.0);
  let nbrSeconds = (sec - (nbrHours * 3600.0)) - (nbrMinutes * 60.0)
  return ("0" + nbrHours).slice(-2) + ":" + ("0" + nbrMinutes).slice(-2) + ":" + ("0" + nbrSeconds).slice(-2);
}

function switchTabToMap()
{
	let visualizationBox = document.getElementById('VisualizationBox');
	visualizationBox.children[0].style.visibility = 'visible';
	visualizationBox.children[1].style.visibility = 'hidden';
}

function updateVisualizationBox() {
  $("body").css("cursor", "wait");
  let metrics = document.getElementsByName('metricSelection');
  let selectedMetric;
  for (var i = 0, length = metrics.length; i < length; i++) {
    if (metrics[i].checked) {
      selectedMetric = metrics[i];
      break;
    }
  }
	if(!selectedMetric)
	{
		return;
	}
	let logName = selectedMetric.getAttribute('data-logName');
	let logUnit = selectedMetric.getAttribute('data-unit');

  let coloredPointsTab = document.getElementById('tabs').getElementsByTagName("a")[0];
  let heatMapTab = document.getElementById('tabs').getElementsByTagName("a")[1];

	let timeValueBegin = NaN;
  let timeValueEnd = NaN;
	let timeValueMin = NaN;
	let timeValueMax = NaN;

  let existTimeline = $('#flat-slider').attr('class') != 'unintialized';
	if(!existTimeline)
	{
		$('#flat-slider').attr('class', 'initialized');
	}
	else {
		timeValueBegin = $('#flat-slider').slider("option", "values")[0];
		timeValueEnd = $('#flat-slider').slider("option", "values")[1];
		timeValueMin = $("#flat-slider").slider("option", "min");
		timeValueMax = $("#flat-slider").slider("option", "max");
	}

	let gradient = []
  let colors = document.getElementById("legendColors").children;
  for (let i = 0, length = colors.length; i < length; i++) {
    gradient.push({
      r: parseInt(colors[i].getAttribute('data-red')),
      g: parseInt(colors[i].getAttribute('data-green')),
      b: parseInt(colors[i].getAttribute('data-blue'))
    });
  }

	let urlLog = !existTimeline
	          ? "/logs/" + logName
						: "/logs/" + logName + "?min=" + secToTimestamp(timeValueBegin) + "&max=" + secToTimestamp(timeValueEnd);

  if (coloredPointsTab.className == "selected") {
    switchTabToMap();
    $.ajax({
      url: urlLog,
      cache: false,
      success: function(log) {
				let minMaxLog = getMinAndMaxOfLog(log, coloredPointsTab);
				let minOfLog = parseInt(minMaxLog[0]);
				let maxOfLog =  Math.ceil(minMaxLog[1]);
				if(!existTimeline)
				{
					updateTimeline(minOfLog, maxOfLog);
				}
				loadColorGradient(minOfLog, maxOfLog, gradient);
				let parsedLog = parseLog(log, logUnit, maxOfLog);
				updateVisualizationLayer(parsedLog, "coloredPoints", gradient);
      }
    });

  } else if (heatMapTab.className == "selected") {
    switchTabToMap();
    $.ajax({
      url: urlLog,
      cache: false,
      success: function(log) {
				let minMaxLog = getMinAndMaxOfLog(log, coloredPointsTab);
				let minOfLog = parseInt(minMaxLog[0]);
				let maxOfLog =  Math.ceil(minMaxLog[1]);
				if(!existTimeline)
				{
					updateTimeline(minOfLog, maxOfLog);
				}
				loadColorGradient(minOfLog, maxOfLog, gradient);
				let parsedLog = parseLog(log, logUnit, maxOfLog);
				updateVisualizationLayer(parsedLog, "heatMap", gradient);
      }
    });
  }


  $("body").css("cursor", "default");
}
