# CurseManifestFiller
Super simple helper tool that helps modpack authors who need to dig in their manifest to find the right project ID when editing the modpack.

Example input:
```json

{
  "minecraft": {
    "version": "1.12.2",
    "modLoaders": [
      {
        "id": "forge-14.23.4.2755",
        "primary": true
      }
    ]
  },
  "manifestType": "minecraftModpack",
  "manifestVersion": 1,
  "files": [
    {
      "projectID": 236199,
      "fileID": 2490951,
      "required": true
    },
    {
      "projectID": 254035,
      "fileID": 2571132,
      "required": true
    },
    {
      "projectID": 235595,
      "fileID": 2616395,
      "required": true
    },
    {
      "projectID": 258426,
      "fileID": 2600448,
      "required": true
    }
  ]
}
```

Example output:
```json

{
  "files": [
    {
      "fileID": 2490951,
      "file_url": "https://minecraft.curseforge.com/projects/missing-pieces/files/2490951",
      "name": "Missing Pieces",
      "projectID": 236199,
      "project_url": "https://minecraft.curseforge.com/projects/missing-pieces",
      "required": true
    },
    {
      "fileID": 2571132,
      "file_url": "https://minecraft.curseforge.com/projects/ender-ore/files/2571132",
      "name": "Ender Ore",
      "projectID": 254035,
      "project_url": "https://minecraft.curseforge.com/projects/ender-ore",
      "required": true
    },
    {
      "fileID": 2616395,
      "file_url": "https://minecraft.curseforge.com/projects/not-enough-wands/files/2616395",
      "name": "Not Enough Wands",
      "projectID": 235595,
      "project_url": "https://minecraft.curseforge.com/projects/not-enough-wands",
      "required": true
    },
    {
      "fileID": 2600448,
      "file_url": "https://minecraft.curseforge.com/projects/forge-multipart-cbe/files/2600448",
      "name": "Forge MultiPart CBE",
      "projectID": 258426,
      "project_url": "https://minecraft.curseforge.com/projects/forge-multipart-cbe",
      "required": true
    }
  ],
  "manifestType": "minecraftModpack",
  "manifestVersion": 1,
  "minecraft": {
    "version": "1.12.2",
    "modLoaders": [
      {
        "id": "forge-14.23.4.2755",
        "primary": true
      }
    ]
  }
}
```

### Important: The result JSON is rearranged!
