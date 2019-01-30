import overpy

overpassApi = overpy.Overpass()


def fetch_city_osm(city):
    return overpassApi.queryRaw("[out:xml];(rel[name='" + city + "'][type=boundary];);out geom;")


