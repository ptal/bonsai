package bonsai.cp.core;

import java.util.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;
import bonsai.runtime.core.*;
import bonsai.runtime.choco.*;
import bonsai.runtime.sugarcubes.*;

public class OneSolution implements Executable
{
  private channel single_time FlatLattice<Consistent> consistent = bot;

  public proc execute() {
    loop {
      when consistent |= Consistent.True {
        exit FoundSolution;
      }
      pause;
    }
  }
}
