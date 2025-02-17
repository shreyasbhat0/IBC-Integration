package ibc.icon.structs.messages;

public class MsgCreateClient {
    private String clientType;
    private byte[] clientState;
    private byte[] consensusState;
    private int btpNetworkId;
    private byte[] storagePrefix;

    public String getClientType() {
        return clientType;
    }

    public void setClientType(String clientType) {
        this.clientType = clientType;
    }

    public byte[] getClientState() {
        return clientState;
    }

    public void setClientState(byte[] clientState) {
        this.clientState = clientState;
    }

    public byte[] getConsensusState() {
        return consensusState;
    }

    public void setConsensusState(byte[] consensusState) {
        this.consensusState = consensusState;
    }

    public int getBtpNetworkId() {
        return btpNetworkId;
    }

    public void setBtpNetworkId(int btpNetworkId) {
        this.btpNetworkId = btpNetworkId;
    }

    public byte[] getStoragePrefix() {
        return storagePrefix;
    }

    public void setStoragePrefix(byte[] storagePrefix) {
        this.storagePrefix = storagePrefix;
    }
}
