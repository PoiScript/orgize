# Orgize-sync

Sync your Org with your favourite applications.

> This project is still in *alpha stage*. Don't forget to backup
> your orgmode files before trying!

## Commands

### `Init`

// TODO

### `Sync`

// TODO

### `Conf`

// TODO

## Configuration

### General

> Field with default value is optional.

**Global**:

``` toml
# path to dotenv file
env_path = "./.env"   # default is `${UserCache}/orgize-sync/.env`

# number of days to filter headline before today
up_days = 1           # default is 7
# number of days to filter headline after today
down_days = 1         # default is 7
```

**Pre-file**:

``` toml
[[file]]
# path to this orgmode file, required
path = "./notes.org"
# specify a name for this file, optional
name = "note"
```

### Google Calendar

**Global**:

``` toml
[google_calendar]
# google oauth client id and client_secret, required
client_id = "xxx"     # or environment variable `GOOGLE_CLIENT_ID`
client_secret = "xxx" # or environment variable `GOOGLE_CLIENT_SECRET`

# redirect url after authorizing
redirect_uri = ""     # default is `http://localhost`

# control where to store access token and refresh token
token_dir = ""        # default is `${UserCache}/orgize-sync`
token_filename = ""   # default is `google-token.json`
```

**Pre-file**:

``` toml
[[file]]
# other fields ...
[file.google_calendar]
# which calendar to sync, required
calendar = ""

# whether to append new calendar event to the org mode
append_new = false    # default is true
# where to append new calendar event
append_headline = ""  # default is `Sync`

# which property to store event id
property = ""         # default is `EVENT_ID`
```
