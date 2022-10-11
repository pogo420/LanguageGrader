# LanguageGrader
* Personal framework to grade a programming language.
* With too many language in market, choosing the right one can be a challenge.

## Personal protocols are listed below:
* For any new language monitor below topics to make decision.

|Supported Topics|Concept|
|-|-|
|Interfaces|OOPS|
|Inhetance or similar|OOPS|
|Encapstulations/Class/Struct/etc |OOPS|
|Encapstulations/Class/Struct methods|OOPS|
|Errors/Exception/CustomExceptions|Exceptions|
|Argument Paser|Frameworks|
|Logger|Frameworks|
|Data format endoders/decoders|Frameworks|
|Concurrency/Async|Frameworks|
|Web - http/sockets/etc|Frameworks|

## Simple project to cover most on the topics:
```
SandwichApp

Sandwich
	// data structure 
	Id
	Name
	Recipe

PersistanceEngine:
	// Master logic for saving the Sandwich

StorageInterfaceReader:
	// leverage json/xml data formats (All language has DB connectors).
	get_sandwich(Name)	

StorageInterfaceWriter:
	// leverage json/xml data formats (All language has DB connectors).
	save_sandwich(Sandwich)	
	update_sandwich(Sandwich, id)

StorageInterfaceReaderWriter
	StorageInterfaceReader
	StorageInterfaceWriter

Exceptions:
	DuplicateSandwichException
	InvalidSandwichData
	SandwichNotFoundException

Additional:
  Add logger in the code flow.
  Command line argumnet to enable logger modes.
  Add unit test cases for each logic.
```
## Concurrency pointers:
* Multi threading frameworks.
* Multi processing frameworks.
* Special concurrent framworks. 
* Exception/recovery support in concurrent flows.

## Web framworks:
* raw web protocol support
* popular web framwroks used.
