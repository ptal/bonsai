package bonsai.cp.core;

import java.util.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;
import bonsai.chococubes.core.*;
import bonsai.chococubes.choco.*;
import bonsai.chococubes.sugarcubes.*;

public class Propagation implements Executable
{
  private channel world_line VarStore domains = bot;
  private channel world_line ConstraintStore constraints = bot;
  private channel single_time FlatLattice<Consistent> consistent = bot;

  public proc execute() {
    loop {
      consistent <- PropagatorEngine.propagate(domains, constraints);
      pause;
    }
  }
}
