Aptos Node Health Checker Deployment
================================

This Helm chart deploys an instance of the Aptos Node Health Checker. NHC can be configured to run a comprehensive suite of health checks based on metrics, the API, and specific node "microservices", such as state sync, consensus, etc.

Configuration
-------------

TODO: Overhaul this readme.

See [values.yaml][] for the full list of options you can configure.

* `chain.name`: Select which blockchain to connect to. Current values:
  - "testnet": Aptos testnet
* `storage.class`: This needs to be set to a StorageClass available in your
  Kubernetes cluster. Example values:
  - AWS: "gp2"
  - GCP: "standard"
  - Azure: "managed"
* `service.type`: By default the REST API endpoint is only exposed within the
  Kubernetes cluster. If you want to expose it externally set this to
  "LoadBalancer".
* `service.loadBalancerSourceRanges`: If you enable the LoadBalancer service you
  can set this to a list of IP ranges to restrict access to.
* `image.tag`: Select the image tag to deploy. For a full list of image tags, check out the [Aptos DockerHub][todo]. You likely want the tag `latest`, the NHC build is network / node type agnostic, you instead configure it with the configurations in `configs/` and select those at request-time.

Connecting to Testnet
-------------

To connect to the Aptos devnet, you must have the correct genesis blob and waypoint. The source of truth for these are hosted here:
* https://devnet.aptoslabs.com/waypoint.txt
* https://devnet.aptoslabs.com/genesis.blob

The waypoint is configured as a helm value in `aptos_chains.devnet.waypoint`, and the genesis blob should be copied to `files/genesis/testnet.blob`

You may also need to change the chain era helm value in `chain.era` to the source of truth hosted at:
* https://devnet.aptoslabs.com/era.txt

Deployment
----------

1. Install Helm v3: https://helm.sh/docs/intro/install/
2. Configure `kubectl` with the Kubernetes cluster you wish to use.
3. Install the release, setting any options:

       $ helm install fullnode --set storage.class=gp2 .

[REST API]: https://github.com/aptos-labs/aptos-core/blob/main/api/doc/openapi.yaml
[values.yaml]: values.yaml
[Aptos dockerhub]: https://hub.docker.com/r/aptoslabs/validator/tags?page=1&ordering=last_updated
