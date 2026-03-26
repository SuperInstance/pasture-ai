# Discord Flow Schema

## Purpose
High-level flow from "Collie" input (e.g., a command or keyword) to generating species JSON output in a Discord bot context.

## Flow Diagram (Text-based)
```
[User Message] --> [Bot Receives: "Collie"] 
                  |
                  v
[Parse Input] --> If keyword "Collie" detected
                  |
                  v
[Query Database/API] --> Fetch species data (e.g., dog breeds, focusing on Collie)
                  |
                  v
[Generate JSON] --> Structure as { "species": "Canine", "breed": "Collie", "traits": ["loyal", "intelligent"], ... }
                  |
                  v
[Reply in Discord] --> Send JSON as formatted message or embed
```

## Pseudocode
```
on_message(message):
    if "Collie" in message.content.lower():
        species_data = fetch_species("Collie")  # e.g., API call or local DB
        json_output = {
            "species": species_data.species,
            "breed": species_data.breed,
            "description": species_data.desc,
            "traits": species_data.traits
        }
        reply(message.channel, json.dumps(json_output, indent=2))
```
