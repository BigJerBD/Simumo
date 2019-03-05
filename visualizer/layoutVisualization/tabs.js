/*
 Tabs
 (c) 2009 By Xul.fr
 Freeware
*/


function loadit( element)
{
	let visualisationBox = document.getElementById('VisualizationBox');
	let tabs= document.getElementById('tabs').getElementsByTagName("a");
	for (var i=0; i < tabs.length; i++)
	{
			tabs[i].className =  tabs[i].rel == element.rel ? "selected" : "";
	}

	updateVisualizationBox();
}
