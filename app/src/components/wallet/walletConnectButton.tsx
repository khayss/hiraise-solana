"use client";

import {
  useWalletModal,
  WalletIcon,
  //   WalletMultiButton,
} from "@solana/wallet-adapter-react-ui";
import { useWalletMultiButton } from "@solana/wallet-adapter-base-ui";
import { Button } from "../ui/button";
import { useEffect, useMemo, useRef, useState } from "react";

const LABELS = {
  "change-wallet": "Change wallet",
  connecting: "Connecting ...",
  "copy-address": "Copy address",
  copied: "Copied",
  disconnect: "Disconnect",
  "has-wallet": "Connect",
  "no-wallet": "Select Wallet",
} as const;

export function WalletConnectButton({
  children,
}: {
  children?: React.ReactNode;
}) {
  const { setVisible: setModalVisible } = useWalletModal();
  const {
    buttonState,
    onConnect,
    onDisconnect,
    publicKey,
    walletIcon,
    walletName,
  } = useWalletMultiButton({
    onSelectWallet() {
      setModalVisible(true);
    },
  });
  const [copied, setCopied] = useState(false);
  const [menuOpen, setMenuOpen] = useState(false);
  const ref = useRef<HTMLUListElement>(null);
  useEffect(() => {
    const listener = (event: MouseEvent | TouchEvent) => {
      const node = ref.current;

      // Do nothing if clicking dropdown or its descendants
      if (!node || node.contains(event.target as Node)) return;

      setMenuOpen(false);
    };

    document.addEventListener("mousedown", listener);
    document.addEventListener("touchstart", listener);

    return () => {
      document.removeEventListener("mousedown", listener);
      document.removeEventListener("touchstart", listener);
    };
  }, []);

  const content = useMemo(() => {
    if (children) {
      return children;
    } else if (publicKey) {
      const base58 = publicKey.toBase58();
      return base58.slice(0, 4) + ".." + base58.slice(-4);
    } else if (buttonState === "connecting" || buttonState === "has-wallet") {
      return LABELS[buttonState];
    } else {
      return LABELS["no-wallet"];
    }
  }, [buttonState, children, publicKey]);
  return (
    <div className="w-max relative">
      <Button
        aria-expanded={menuOpen}
        style={{ pointerEvents: menuOpen ? "none" : "auto" }}
        onClick={() => {
          switch (buttonState) {
            case "no-wallet":
              setModalVisible(true);
              break;
            case "has-wallet":
              if (onConnect) {
                onConnect();
              }
              break;
            case "connected":
              setMenuOpen(true);
              break;
          }
        }}
        className="flex items-center justify-center gap-2"
      >
        {walletName && walletIcon && (
          <span className="h-full">
            <WalletIcon
              className="object-contain h-full"
              wallet={{ adapter: { icon: walletIcon, name: walletName } }}
            />
          </span>
        )}
        <span>{content}</span>
      </Button>
      <ul
        aria-label="dropdown-list"
        className={`absolute flex flex-col bg-muted gap-2 p-2 rounded-md mt-2 ${
          !menuOpen && "hidden"
        }`}
        ref={ref}
        role="menu"
      >
        {publicKey ? (
          <li role="menuitem">
            <Button
              className="w-full"
              onClick={async () => {
                await navigator.clipboard.writeText(publicKey.toBase58());
                setCopied(true);
                setTimeout(() => setCopied(false), 400);
              }}
            >
              {copied ? LABELS["copied"] : LABELS["copy-address"]}
            </Button>
          </li>
        ) : null}
        <li role="menuitem">
          <Button
            className="w-full"
            onClick={() => {
              setModalVisible(true);
              setMenuOpen(false);
            }}
          >
            {LABELS["change-wallet"]}
          </Button>
        </li>
        {onDisconnect ? (
          <li role="menuitem">
            <Button
              className="w-full"
              onClick={() => {
                onDisconnect();
                setMenuOpen(false);
              }}
            >
              {LABELS["disconnect"]}
            </Button>
          </li>
        ) : null}
      </ul>
    </div>
  );
}
