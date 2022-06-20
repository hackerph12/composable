import { useEffect } from "react";
import { Assets } from "@/defi/polkadot/Assets";
import { AssetId } from "@/defi/polkadot/types";
import {
  useExtrinsics,
  useParachainApi,
  useSelectedAccount,
} from "substrate-react";
import useStore from "@/store/useStore";
import { fetchBalanceByAssetId } from "@/defi/utils";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import _ from "lodash";

const processedTransactions: string[] = [];
const Updater = () => {
  const { updateAssetBalance } = useStore();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const extrinsicCalls = useExtrinsics();

  useEffect(() => {
    if (parachainApi && selectedAccount) {
      Object.keys(Assets).forEach((asset) => {
        let assetID: string | number | null =
          Assets[asset as AssetId].supportedNetwork[DEFAULT_NETWORK_ID];
        if (assetID) {
          assetID = assetID.toString();
          fetchBalanceByAssetId(
            parachainApi,
            selectedAccount.address,
            assetID
          ).then((balance) => {
            updateAssetBalance(asset as AssetId, DEFAULT_NETWORK_ID, balance);
          });
        }
      });
    }
  }, [parachainApi, selectedAccount, updateAssetBalance]);

  useEffect(() => {
    if (
      parachainApi &&
      selectedAccount &&
      Object.values(extrinsicCalls).length > 0
    ) {
      const txs = Object.values(extrinsicCalls);

      let shouldUpdate: string | null = null;
      txs.forEach((tx) => {
        if (
          tx.sender === selectedAccount.address &&
          tx.status === "isFinalized" &&
          (tx.section === "dexRouter" || tx.section === "pablo") &&
          !processedTransactions.includes(tx.hash)
        ) {
          shouldUpdate = tx.hash;
        }
      });

      if (shouldUpdate !== null) {
        const allPromises = Object.keys(Assets).map((asset) => {
          return new Promise((res, rej) => {
            let assetID: string | number | null =
              Assets[asset as AssetId].supportedNetwork[DEFAULT_NETWORK_ID];
            if (assetID) {
              assetID = assetID.toString();
              fetchBalanceByAssetId(
                parachainApi,
                selectedAccount.address,
                assetID
              ).then((balance) => {
                updateAssetBalance(asset as AssetId, DEFAULT_NETWORK_ID, balance);
                res(asset);
              });
            } else {
              res(asset);
            }
          });
        });

        Promise.all(allPromises).then((updatedBalancesAssetList) => {
          processedTransactions.push(shouldUpdate as string);
        });
      }
    }
  }, [extrinsicCalls, parachainApi, selectedAccount, updateAssetBalance]);

  return null;
};

export default Updater;
