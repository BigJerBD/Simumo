import jinja2
from distutils.dir_util import copy_tree
import os
import shutil


def render_osm_map(city):
    # copy directory containing the outputmap and its ressources
    geocoder_output_path = "./ol-geocoder/template"
    map_output_path = "./output"
    shutil.rmtree(map_output_path) #in case the command was run before
    copy_tree(geocoder_output_path, map_output_path)

    # generate javascript file from template so that
    # the map generated is zoomed at the requested city
    map_template = map_output_path + "/OlMapTemplate.j2"
    template_loader = jinja2.FileSystemLoader(searchpath="./")
    template_env = jinja2.Environment(loader=template_loader)
    template = template_env.get_template(map_template)
    map_rendered = template.render(initialLocation="\"" + city + "\"")
    with open(map_output_path + "/OlMapTemplate.js", "w") as text_file:
        print(map_rendered, file=text_file)
    os.remove(map_output_path + "/OlMapTemplate.j2")
    os.remove(map_output_path + "/ol-geocoder.css")
    os.remove(map_output_path + "/ol-geocoder.min.css")



render_osm_map("Montreal")
