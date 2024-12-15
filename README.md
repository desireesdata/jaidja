> 🚧 🏗️ En travaux !

# jaidja : programme de détection des doublons ou des images similaires

*jaidja* est un programme exécutable en ligne de commande qui permet de détecter les images en double ou similiaires dans un répertoire.

![](/src/img/demo.gif)

### Détection des doublons

La détection des doublons s'effectue par l'obtention d'empruntes des fichiers images avec des algorithmes de hashage cryptographiques. Deux images de noms différents peuvent avoir le même contenu. Si deux images sont, au bit près, identiques, alors elles sont considérées comme des doublons. C'est une approche comparative "stricte". 

> Le moindre pixel différent impliquera une emprunte différente. L'avantage de cette méthode est détecter (et donc de supprimer) une image si et seulement si elles sont parfaitement identiques : c'est un bon garde fou à certaines erreurs de manipulations.

```bash
jaidja exact jpg # Vérifie l'existence de doublons dans tous les jpg du dossier
```

On peut diriger les résultats sur un fichier de cette façon :

```bash
jaidja jpg > doublons.txt
```

En attente de features de suppression sécurisées, on peut utiliser, **avec beaucoup de prudence** cette commande :

```bash
rm *.jpg | jaidja doublons jpg
```

### Détection des similarités

La détection des similarités repose sur l'utilisation de *hashs perceptuels*. Cela veut dire que deux images qui ont subi des modifications mineures ou réversibles (redimensionnement, contraste...), même si elles impliquent des trains de bits différents, sont reconnues comme similiaires. Un seuil de tolérance (pour l'instant arbitraire) déterminé à partir de la distance de Hamming sur les hashes, doit être appliqué.

```bash
jaidja similar jpg > similarities.txt
```

## Pourquoi Rust ?

Ce programme, à améliorer, est conçu comme s'il avait pour destination d'être utilisé pour le traitement industriel de grands lots d'images au sein d'institutions patrimoniales. Rust, rapide et offrant un meilleur contrôle sur la mémoire, semble donc un choix approprié. 

## Installer *jaidja*

> A rédiger !

## Aller plus loin

- Parcourir une arborescence entière

- Traitements parallèles

- Feature de suppression sécurisée (affichage des distances)

- Améliorer la gestion des erreurs

- Prévisualisation des images détectées comme similaires

- Pouvoir supprimer directement le doublon le moins récent

- Créer des options sur supprimer le fichier le moins ou le plus lourd

- Génération d'un bordereau des détections et des suppressions horodaté et lui-même hashé pour archivage
