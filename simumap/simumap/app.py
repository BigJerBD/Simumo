from osm_graph import OsmGraph
from overpy.exception import OverPyException

__GRAPH__ = None



def target_location(lon, lat, zoom):
    """
    set the location to get info from the OSM Graph
    :return: True | False
    """
    global __GRAPH__
    try:
        __GRAPH__ = OsmGraph(lon, lat, zoom)
    except OverPyException:
        raise Exception("Can't query graph from targeted location")


def untarget_location():
    """
    discard the current coordinate target
    :return:
    """
    global __GRAPH__
    __GRAPH__ = None


def get_nodes():
    """
    get the nodes ID with their position from the seted location
    :return: {ID:(x,y)}
    """
    global __GRAPH__
    if __GRAPH__:
        return __GRAPH__.pos
    else:
        return []


def get_edges():
    """
    get the edges between nodes in a dictionary
    :return: {ID:ID}
    """
    if __GRAPH__:
        return __GRAPH__.graph.edges
    else:
        return []


def get_adjacencies():
    """
    get a mapping of nodes to a list of their adjacent nodes
    :return: {ID:[ID]}
    """
    if __GRAPH__:
        return {
            node: [
                child for child in next_nodes.keys()
            ]
            for node, next_nodes in __GRAPH__.graph.adj.items()
        }
    else:
        return []
