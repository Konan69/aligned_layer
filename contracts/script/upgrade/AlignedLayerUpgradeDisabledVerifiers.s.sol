// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {Script} from "forge-std/Script.sol";
import "eigenlayer-contracts/src/contracts/core/AVSDirectory.sol";
import {RegistryCoordinator} from "eigenlayer-middleware/RegistryCoordinator.sol";
import {StakeRegistry} from "eigenlayer-middleware/StakeRegistry.sol";
import {IRewardsCoordinator} from "eigenlayer-contracts/src/contracts/interfaces/IRewardsCoordinator.sol";
import {AlignedLayerServiceManager} from "src/core/AlignedLayerServiceManager.sol";
import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import "forge-std/StdJson.sol";

contract AlignedLayerSetAggregator is Script {
    function run(
        string memory eigenLayerDeploymentFilePath,
        string memory alignedLayerDeploymentFilePath,
        string memory alignedConfigFilePath
    ) external returns (address, address) {
        string memory eigen_deployment_file = vm.readFile(
            eigenLayerDeploymentFilePath
        );

        string memory aligned_deployment_file = vm.readFile(
            alignedLayerDeploymentFilePath
        );

        string memory aligned_config_file = vm.readFile(alignedConfigFilePath);

        ProxyAdmin alignedLayerProxyAdmin = ProxyAdmin(
            stdJson.readAddress(
                aligned_deployment_file,
                ".addresses.alignedLayerProxyAdmin"
            )
        );

        RegistryCoordinator registryCoordinator = RegistryCoordinator(
            stdJson.readAddress(
                aligned_deployment_file,
                ".addresses.registryCoordinator"
            )
        );

        AVSDirectory avsDirectory = AVSDirectory(
            stdJson.readAddress(
                eigen_deployment_file,
                ".addresses.avsDirectory"
            )
        );

        StakeRegistry stakeRegistry = StakeRegistry(
            stdJson.readAddress(
                aligned_deployment_file,
                ".addresses.stakeRegistry"
            )
        );

        IRewardsCoordinator rewardsCoordinator = IRewardsCoordinator(
            stdJson.readAddress(
                eigen_deployment_file,
                ".addresses.rewardsCoordinator"
            )
        );

        vm.startBroadcast();

        AlignedLayerServiceManager alignedLayerServiceManagerImplementation = new AlignedLayerServiceManager(
                avsDirectory,
                rewardsCoordinator,
                registryCoordinator,
                stakeRegistry
            );

        vm.stopBroadcast();
        vm.startBroadcast();

        // alignedLayerServiceManager is the proxy
        AlignedLayerServiceManager alignedLayerServiceManager = AlignedLayerServiceManager(
                payable(
                    stdJson.readAddress(
                        aligned_deployment_file,
                        ".addresses.alignedLayerServiceManager"
                    )
                )
            );

        vm.stopBroadcast();
        vm.startBroadcast();

        alignedLayerProxyAdmin.upgrade(
            TransparentUpgradeableProxy(
                payable(address(alignedLayerServiceManager))
            ),
            address(alignedLayerServiceManagerImplementation)
        );

        vm.stopBroadcast();
        vm.startBroadcast();

        alignedLayerServiceManager.setDisabledVerifiers(0);

        vm.stopBroadcast();

        return (
            address(alignedLayerServiceManager),
            address(alignedLayerServiceManagerImplementation)
        );
    }
}
