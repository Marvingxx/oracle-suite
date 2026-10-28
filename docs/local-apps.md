# Applications locales

Le lanceur affiche aussi les applications existantes enregistrées dans
`%LOCALAPPDATA%/Oracle Suite/local-apps.json` (ou le dossier `data` en mode portable).
Elles apparaissent dans Applications et dans les recherches, avec le bouton Ouvrir.
Le lanceur démarre directement leur exécutable dans son dossier, sans shell.
Ces applications ne participent pas aux téléchargements, mises à jour ou désinstallations du catalogue distant.

```json
[
  {
    "id": "pratique",
    "name": "Pratique",
    "description": "Tutoriels, apprentissage et synthèses personnelles",
    "executable": "C:\\Users\\USERNAME\\AppData\\Local\\Programs\\Pratique\\Pratique.exe"
  }
]
```

Les chemins doivent être absolus et désigner un fichier existant (`.exe` sous Windows).
Les entrées invalides, les doublons et les identifiants déjà réservés au catalogue sont ignorés.
Le fichier est relu au démarrage du lanceur. Pratique conserve son installation,
son profil et son dossier de données ; aucun contenu n'est copié dans une seconde bibliothèque.

Oracle Note existe séparément dans `../oracle-note` : prototype web local v0.1,
avec documents, notes, moodboards et sauvegardes JSON. Son point d'entrée est
`oracle-note/LANCER.cmd`, sur le port 5186. Il n'a pas encore d'installateur natif
ni d'intégration à la Bibliothèque commune.
