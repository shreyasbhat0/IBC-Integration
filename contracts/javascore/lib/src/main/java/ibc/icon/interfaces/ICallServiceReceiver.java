/*
 * Copyright 2022 ICON Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package ibc.icon.interfaces;

import foundation.icon.score.client.ScoreInterface;
import score.annotation.External;

@ScoreInterface
public interface ICallServiceReceiver {

    /**
     * Handles the call message received from the source chain.
     * Only called from the Call Message Service.
     *
     * @param _from The BTP address of the caller on the source chain
     * @param _data The calldata delivered from the caller
     */
    @External
    void handleCallMessage(String _from, byte[] _data);
}
