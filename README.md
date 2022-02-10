# Downloads Sorter

This is a simple CLI tool to help you manage your downloads.

After installing, make a downloads_sorter.toml file in ```$HOME/.config/downloads_sorter.toml```. 

Change the behavior in the downloads_sorter.toml file. 
Here is a sample: 
```
sortingdir = 'none'
archivedir = 'none'

[[folderconfig]]
name = 'homework'
keywords = ['hw', 'problemset', 'project', 'lecture' ]
matchertypes = []

[[folderconfig]]
name = 'installers'
keywords = ['amd', 'msi', 'installer']
matchertypes = ['app']
```

The sortingdir is where the program will look for files to sort, default is the downloads folder.

The archivedir is the base directory of where the program will create the folders and then sort the files.

The name of the folderconfig entry will be the name of the folder in the archive. The keywords will be checked against file names and then sent to that folder. The matchertypes will be MIME types. These do not work very well right now, so it is recommended to just use keywords.

Valid matcher types are:
```
['app', 'archive', 'audio', 'document', 'font', 'image', 'text' ]
```

Plans:
Regex catgerization of files.
Terminal menu.