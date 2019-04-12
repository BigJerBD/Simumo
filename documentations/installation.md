# Manuel d'installation #
Ce manuel explique comment installer le simulateur ainsi que le visualisateur de **Simumo**.


## Prérequis ##
* Python 3.6.7 https://www.python.org/downloads/release/python-367/
* make
* Rustup

## Installation ##

### développement ##
Afin de créer votre environnement virtuel de développement, exécuter la commande à partir de la racine du dossier dans le terminal:
```
make dev_venv
```
ensuite pour entrer dans l'environnement virtuel, exécuter:
```
source venv/Scripts/activate
```

Pour sortir de l'envrionnement, exécuter:
```
deactivate
```
#### Simulateur ####
Pour éxecuter le simulateur,  exécuter la commande à partir de la racine du dossier **simulator** dans le terminal:
```
cargo run -- -c <pathOfSimulatorConfigFile>
```
Explorer le dossier *simulator/etc* pour avoir quelques exemples.
#### Visualisateur ####
Pour éxecuter le simulateur,  exécuter la commande à partir de la racine du dossier **visualizer** dans le terminal:
```
python Server.py <pathOfVisualizerConfigFile>
```
Explorer le fichier *visualizer/visualizationConfigExample.yaml* pour avoir un exemple.

### Release ###
Afin de créer l'exécutable exécuter la commande à partir de la racine du dossier dans le terminal:
```
make build
```

Pour avoir plus d'information comment exécuter le simulateur, référez-vous au document: **usage**
