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

package ibc.xcall.integration;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;

import foundation.icon.btp.xcall.data.CSMessageRequest;
import foundation.icon.btp.xcall.data.CSMessageResponse;

public class AssertCallService {
    public static void assertEqualsCSMessageRequest(CSMessageRequest exp, CSMessageRequest got) {
        assertEquals(exp.getFrom(), got.getFrom());
        assertEquals(exp.getTo(), got.getTo());
        assertArrayEquals(exp.getData(), got.getData());
    }

    public static void assertEqualsCSMessageResponse(CSMessageResponse exp, CSMessageResponse got) {
        assertEquals(exp.getSn(), got.getSn());
        assertEquals(exp.getCode(), got.getCode());
        assertEquals(exp.getMsg(), got.getMsg());
    }
}
