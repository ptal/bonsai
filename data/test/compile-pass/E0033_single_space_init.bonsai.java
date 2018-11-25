// Copyright 2018 Pierre Talbot

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package test;

// In contrast to E0033_single_time_init, initialization conditions on single_space and world_line are only important in their first instant.
public class E0033_single_space_init
{
  public proc initSingleSpaceFromSingleSpace() =
    single_space LMax b = new LMax(1);
    single_space LMax a = new LMax(read b);
    pause;
    readwrite b.inc();
  end

  public proc initSingleSpaceFromWorldLine() =
    world_line LMax b = new LMax(1);
    single_space LMax a = new LMax(read b);
    pause;
    readwrite b.inc();
  end

  public proc initSingleSpaceFromSingleTime() =
    single_time LMax b = new LMax(1);
    single_space LMax a = new LMax(read b);
    pause;
    readwrite b.inc();
  end

  public proc initWorldLineFromSingleSpace() =
    single_space LMax b = new LMax(1);
    world_line LMax a = new LMax(read b);
    pause;
    readwrite b.inc();
  end

  public proc initWorldLineFromWorldLine() =
    world_line LMax b = new LMax(1);
    world_line LMax a = new LMax(read b);
    pause;
    readwrite b.inc();
  end

  public proc initWorldLineFromSingleTime() =
    single_time LMax b = new LMax(1);
    world_line LMax a = new LMax(read b);
    pause;
    readwrite b.inc();
  end
}
