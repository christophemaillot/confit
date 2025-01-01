# Confit: In-Place Config File Editor

**Confit** is a command-line tool designed to edit configuration files in place. 


## Installation

**Confit** is written in rust language. To install this tool, you need the ``cargo`` 
command to be available.

``cargo install https://github.com/christophemaillot/confit``

## Usage

### setting a key/value pair

```shell
confit <filename> set <key> <value>
```

Override an existing key/value pair if key is found, or create a new one at the end of the file.

### setting a random config line

To set a random line in the config line, **config** needs to known if a previous value
of this config line exists. **config** uses a marker for this.

The marker is (generally speaking) a comment with a unique identifier.

#### Example 

Original pg_hba.conf file :
```
local   all             all                                     trust

## custom-001
host    postgres        all             192.168.12.10/32        scram-sha-256
```

The marker here is  ``## custom-001``

```
confit pg_hba.conf insert '## custom-001'  'host    postgres        all             192.168.0.00/24        scram-sha-256'
```

will turn the pg_hba.conf file to :
```
local   all             all                                     trust

## custom-001
host    postgres        all             192.168.0.00/24        scram-sha-256
```




