from flask import Flask, render_template, request
import RenderOsmMap
from ParseLogs import get_logs_in_range

app = Flask(__name__,
            static_url_path="",
            static_folder="output",
            template_folder="output")


@app.route('/')
def send_visualization_layout():
    return render_template('layout.html')


@app.route('/logs/<metric>')
def send_metric(metric):
    min = request.args.get('min')
    max = request.args.get('max')
    return get_logs_in_range(metric, min, max)


if __name__ == "__main__":
    timeline = {"min": 0, "max": 110}
    metrics = [
        {
            "name" : "Sample_Sherbrooke",
            "logName" : "sherbrooke_sample.ndjson",
            "unit" : "MeterPerSecond",
            "unitLabel" : "mètre par seconde"
        }
    ]
    # legend = [
    #     {"red": 255,"green": 255,"blue" :0},
    #     {"red": 255, "green": 128, "blue": 0},
    #     {"red": 255, "green": 0, "blue": 0}
    # ]
    legend = [
        {"red": 0, "green": 255, "blue": 0},
        {"red": 255, "green": 255, "blue": 0},
        {"red": 255, "green": 0, "blue": 0}
    ]
    RenderOsmMap.render_visualization("Sherbrooke", timeline, metrics, legend)
    app.run("0.0.0.0")
