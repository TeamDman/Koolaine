using System.Windows;
using System;
using System.Collections.Generic;
using System.Windows.Automation;

namespace MyCursorApp
{
    class Program
    {
        static void Main(string[] args)
        {
            System.Windows.Point point = new System.Windows.Point(500, 500);
            AutomationElement element = AutomationElement.FromPoint(point);

            if (element != null)
            {
                Console.WriteLine($"Element found at point {point}: {element.Current.Name}, {element.Current.ControlType.ProgrammaticName}");

                Console.WriteLine("=== Gather Interesting ===");
                foreach (var item in GatherInteresting(element))
                {
                    (AutomationElement elem, int depth, string relation) = item;
                    Rect boundingRect = (Rect)elem.GetCurrentPropertyValue(AutomationElement.BoundingRectangleProperty);
                    Console.WriteLine($"{relation}: {elem.Current.Name}, Depth: {depth}, BoundingRect: {boundingRect}");
                }
            }
            else
            {
                Console.WriteLine("No element found at the given coordinates.");
            }
        }

        static IEnumerable<(AutomationElement, int, string)> GatherInteresting(AutomationElement element, int maxDepth = 2)
        {
            // Yield children of the element
            foreach (var child in GetDescendants(element, 0, maxDepth))
            {
                yield return (child.Item1, child.Item2, "Child");
            }

            // Yield siblings of the element
            TreeWalker walker = TreeWalker.ControlViewWalker;
            AutomationElement sibling = walker.GetNextSibling(element);
            while (sibling != null)
            {
                yield return (sibling, 0, "Sibling");
                sibling = walker.GetNextSibling(sibling);
            }

            // Yield siblings of the element's parent
            AutomationElement parent = walker.GetParent(element);
            if (parent != null)
            {
                AutomationElement parentSibling = walker.GetNextSibling(parent);
                while (parentSibling != null)
                {
                    yield return (parentSibling, 0, "Parent's Sibling");
                    parentSibling = walker.GetNextSibling(parentSibling);
                }
            }
        }

        static IEnumerable<AutomationElement> GetAncestors(AutomationElement element)
        {
            TreeWalker walker = TreeWalker.ControlViewWalker;
            AutomationElement parentElement = walker.GetParent(element);

            while (parentElement != null)
            {
                yield return parentElement;
                parentElement = walker.GetParent(parentElement);
            }
        }

        static IEnumerable<(AutomationElement, int)> GetDescendants(AutomationElement element, int currentDepth, int maxDepth)
        {
            if (currentDepth > maxDepth)
            {
                yield break;
            }

            TreeWalker walker = TreeWalker.ControlViewWalker;
            AutomationElement childElement = walker.GetFirstChild(element);

            while (childElement != null)
            {
                yield return (childElement, currentDepth);

                foreach (var grandChild in GetDescendants(childElement, currentDepth + 1, maxDepth))
                {
                    yield return grandChild;
                }

                childElement = walker.GetNextSibling(childElement);
            }
        }
    }
}
