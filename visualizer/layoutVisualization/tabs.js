/*
 Tabs
 (c) 2009 By Xul.fr
 Freeware
*/


function loadit( element)
{
	var visualisationBox = document.getElementById('VisualizationBox');
	var tabs= document.getElementById('tabs').getElementsByTagName("a");
	for (var i=0; i < tabs.length; i++)
	{
		if(tabs[i].rel == element.rel)
			tabs[i].className="selected";
		else
			tabs[i].className="";
	}

	updateVisualizationBox();
}
