(function (win, doc) {

  var olview = new ol.View({
        center: [0, 0],
        zoom: 3,
        minZoom: 13,
        maxZoom: 20
      }),
      baseLayer = new ol.layer.Tile({
        source: new ol.source.OSM()
      }),
      map = new ol.Map({
        target: doc.getElementById('map'),
        view: olview,
        layers: [baseLayer]
      }),
      popup = new ol.Overlay.Popup();

  //Instantiate with some options and add the Control
  var geocoder = new Geocoder('nominatim', {
    provider: 'photon',
    targetType: 'glass-button',
    lang: 'en',
    placeholder: 'Search for ...',
    limit: 5,
    keepOpen: false,
    initialLocation: "Montreal"
  });

  map.addControl(geocoder);
  map.addOverlay(popup);



})(window, document);
