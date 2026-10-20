#!/usr/bin/env python3
"""Render the chart and verify network isolation. Requires Helm and PyYAML."""
import ipaddress
import json
from pathlib import Path
import subprocess
import tempfile

import yaml

CHART = Path(__file__).resolve().parents[1] / "helm/check-if-email-exists"


def render(values, extra=None, succeeds=True):
    with tempfile.NamedTemporaryFile(mode="w", suffix=".json") as overlay:
        json.dump(extra or {}, overlay)
        overlay.flush()
        result = subprocess.run(
            ["helm", "template", "audit", str(CHART), "--namespace", "audit",
             "-f", str(CHART / values), "-f", overlay.name],
            capture_output=True, text=True,
        )
    assert (result.returncode == 0) == succeeds, result.stderr
    return list(yaml.safe_load_all(result.stdout)) if succeeds else []


def allows(policy, address, port):
    address = ipaddress.ip_address(address)
    for rule in policy["spec"]["egress"]:
        assert rule.get("to"), "An unrestricted egress rule defeats isolation"
        ports = rule.get("ports")
        if ports and not any(p["port"] == port and p["protocol"] == "TCP" for p in ports):
            continue
        for peer in rule["to"]:
            block = peer.get("ipBlock")
            if block and address in ipaddress.ip_network(block["cidr"]):
                if not any(address in ipaddress.ip_network(cidr) for cidr in block.get("except", [])):
                    return True
    return False


for values in ("values.yaml", "values.staging.yaml", "values.production.yaml"):
    docs = render(values, {"smtpMonitoring": {"enabled": True}})
    policy = next(d for d in docs if d and d["kind"] == "NetworkPolicy")
    deployment = next(d for d in docs if d and d["kind"] == "Deployment")
    assert policy["spec"]["podSelector"] == deployment["spec"]["selector"]
    assert policy["spec"]["policyTypes"] == ["Egress"]
    dns = policy["spec"]["egress"][0]
    assert dns["to"] == [{
        "namespaceSelector": {"matchLabels": {"kubernetes.io/metadata.name": "kube-system"}},
        "podSelector": {"matchLabels": {"k8s-app": "kube-dns"}},
    }]
    assert {(p["protocol"], p["port"]) for p in dns["ports"]} == {("UDP", 53), ("TCP", 53)}
    research_rules = [rule for rule in policy["spec"]["egress"]
                      if any("podSelector" in peer and "namespaceSelector" not in peer
                             for peer in rule["to"])]
    assert research_rules == ([] if values == "values.yaml" else [{
        "to": [{"podSelector": {"matchLabels": {
            "app.kubernetes.io/name": "leads-scraper-service",
            "app.kubernetes.io/instance": "lead-scraper-service",
            "app.kubernetes.io/component": "http",
        }}}],
        "ports": [{"protocol": "TCP", "port": 8080}],
    }])
    for address in (
        "0.1.2.3", "10.1.2.3", "100.64.1.1", "127.0.0.1", "169.254.169.254",
        "172.31.1.1", "192.168.1.1", "192.0.0.1", "192.0.2.1", "192.88.99.1",
        "198.19.1.1", "198.51.100.1", "203.0.113.1", "224.0.0.1", "240.0.0.1",
        "::1", "::ffff:127.0.0.1", "64:ff9b::a00:1", "2001::1", "2001:db8::1",
        "2002:a00:1::1", "3fff::1", "fc00::1", "fe80::1",
    ):
        assert not allows(policy, address, 25), (values, address)
        assert not allows(policy, address, 443), (values, address)
        assert not allows(policy, address, 8080), (values, address)
    for address in ("8.8.8.8", "2607:f8b0:400e:c00::1a"):
        assert allows(policy, address, 25)
        assert allows(policy, address, 443)
    monitor = next(d for d in docs if d and d["kind"] == "PodMonitor")
    assert monitor["spec"]["selector"] == deployment["spec"]["selector"]
    assert monitor["spec"]["podMetricsEndpoints"][0]["port"] in {
        p["name"] for c in deployment["spec"]["template"]["spec"]["containers"] for p in c["ports"]
    }
    assert any(d and d["kind"] == "PrometheusRule" for d in docs)

docs = render("values.yaml", {"networkPolicy": {"privateDestinations": [
    {"cidr": "10.1.2.3/32", "ports": [5432]},
]}})
policy = next(d for d in docs if d and d["kind"] == "NetworkPolicy")
assert allows(policy, "10.1.2.3", 5432)
assert not allows(policy, "10.1.2.3", 25)
assert not allows(policy, "10.1.2.4", 5432)
for destination in ({"cidr": "10.0.0.0/8", "ports": [5432]}, {"cidr": "10.1.2.3/32"}):
    render("values.yaml", {"networkPolicy": {"privateDestinations": [destination]}}, succeeds=False)
print("Egress isolation, dependency exceptions, DNS and monitoring renders passed.")
