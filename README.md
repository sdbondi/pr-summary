A quick-and-dirty tool to generate a markdown table that summarizes the status (conflicts, CI, review state) of PRs 

PR                                                             | Author          | Mergeable | Review state        
-------------------------------------------------------------- | --------------- | --------- | ------------------- 
[#912](https://github.com/owner/repo/pull/912) 🔴 | sdbondi         | No        | Needs review        
[#911](https://github.com/owner/repo/pull/911) 🔴 | sdbondi         | No        | Needs review        
[#893](https://github.com/owner/repo/pull/893) 🔴 | mrx       | Conflicts | Approved by sdbondi 
[#895](https://github.com/owner/repo/pull/895) 🟢 | x           | Conflicts | Dismissed           

## Permissions

1. Head to https://github.com/settings/tokens?type=beta
2. Setup these permissions ![image](https://github.com/sdbondi/pr-summary/assets/1057902/d4c1e882-361a-46b6-9ef2-6b680e375c7f)

3. Set `GITHUB_TOKEN` to your personal token, or pass in `--token github_xxxx` on the command line


## Command options

```
Usage: pr-summary --owner <OWNER> --repo <REPO> --personal-token <PERSONAL_TOKEN>

Options:
  -o, --owner <OWNER>                    
  -r, --repo <REPO>                      
  -t, --personal-token <PERSONAL_TOKEN>  [env: GITHUB_TOKEN]
  -h, --help                             Print help
```
