# pubkey

A simple dockerized rust application that serves public keys on a webserver, akin to how you would be able to access your Github account's public keys via:
`https://github.com/<username>.keys`

By mounting a folder containing n pubkey files, this app will concat the files into one output and serve it.

## Usage

Simply build and run the container like so:

```
docker build -t pubkey .

docker run -dit --rm -p 8080:8080 -v <path to share containing pubkeys>:/usr/src/app/keys pubkey
```

Then visit the site at:

`http://localhost:8080`

## Use Cases

I personally use this to help provision my Raspberry Pis. When setting them up, I'd like a quick way to get all my public keys without having the hassle of mounting my NAS to my Pis at the beginning, nor having to connect to the internet and rely on Github for my pubkeys. I don't want to necesarily put my home network pubkets on Github either. 